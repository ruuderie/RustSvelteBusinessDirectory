# Phase 2 Completion: Tenant Provisioning and RBAC Migration

This phase focused on finalizing the transition from the legacy `is_admin` boolean flag to a secure, granular, role-based access control (RBAC) model. 

## Key Changes Made

### 1. Database & Schema Migration
- Extended the `UserRole` enum within `user_account.rs` to include a new `PlatformSuperAdmin` variant, enabling distinct permission levels that supersede tenant-scoped roles.
- Modified the `user.rs` active model to completely remove the deprecated `is_admin` boolean, moving authorization away from the user table to isolated role mappings.

### 2. Authorization Middleware Overhaul
- **`src/middleware/middleware.rs`**: Replaced simple checks on `user.is_admin` with a structured `is_platform_admin` verification. For routes under `/api/admin`, the middleware now queries the `user_account` entity to see if the user possesses the `PlatformSuperAdmin` role.
- Populated the `app_permissions` context dynamically within the auth middleware by fetching associated `user_app_permission` entities and injecting them into the Axum request extensions for downstream handlers.

### 3. Application-Specific Permissions Definition
Created isolated permission enums to strictly define application-level interactions, separating them from platform-level roles:
- **`AnchorPermission` (`src/atlas_apps/anchor.rs`)**: `ManageContent`, `ManageSettings`
- **`NetworkPermission` (`src/atlas_apps/network_instance.rs`)**: `ListingPoster`, `Subscriber`

### 4. Handler Stabilization & Clean Up
- **`src/handlers/listings.rs`**: Transitioned listing creation, modification, and deletion access controls away from `is_admin`. Listing mutation now strictly verifies whether the current user holds an associated `user_account` matching the target profile's `account_id`, falling back securely to a `PlatformSuperAdmin` check for administrative overrides.
- **`src/handlers/tenant.rs`**: Ensured that the `provision_admin` workflow appropriately provisions new users with the `UserRole::Owner` role inside their initial `user_account` rather than flipping a deprecated global boolean.
- **`src/auth.rs`**: Stripped out `is_admin` propagation into encoded JWT claims to prevent token staleness from bypassing security constraints; token integrity now solely relies on runtime database verification for super-admin privileges.

## Verification
- All backend routes compile cleanly with `cargo check`.
- Safe removal of the global admin override guarantees that applications enforcing the `auth_middleware` will only pass authenticated requests bearing verified, contextually-appropriate permissions.
