# Unified Auth Phase 1 Complete: User Provisioning

Phase 1 of the Unified Auth migration has been fully implemented, focusing on the infrastructure required to bootstrap new administrators without passwords.

## Summary of Changes

### 1. `provision-admin` Endpoint
Created a new `POST /api/tenants/{id}/provision-admin` endpoint on the backend. This endpoint handles the secure creation of the initial tenant owner.
- Uses `AuthService::create_setup_token()` to generate a secure, 24-hour token.
- Returns a `setup_url` that the platform administrator can hand to the new tenant owner.

### 2. Wired into `TenantOnboarding`
Updated the `OnboardingWizard` inside the `platform-admin` application. Once all required setup steps are completed, the wizard now prompts the platform administrator to provision the initial tenant owner.

- A quick inline form captures the owner's Name and Email.
- Submitting the form calls `provision-admin` and immediately displays the unique setup link to securely share.

### 3. Setup Token Exchange Route
Added `POST /api/auth/setup/exchange` which accepts the `setup_token` and trades it for a fully verified session (setting the `HttpOnly` session cookie). This is implemented as an alias to the magic link verification flow to ensure consistency in audit logging and session generation.

### 4. Passkey Registration UI (`anchor-app`)
Added the `/setup-passkey` route to the `anchor` application.
- Extracts the setup token from the URL (`?token=...`).
- Exchanges it for a session.
- Invokes the WebAuthn API (`@simplewebauthn/browser`) to securely register a new hardware/software passkey (Face ID, Touch ID, YubiKey).
- Marks the user as fully onboarded.

## Verification
- Backend compiles cleanly (`cargo check`).
- The `anchor` legacy application compiles cleanly with the new Leptos 0.6 compliant UI.
- `platform-admin` compiles and correctly implements the `provision-admin` form.

> [!NOTE]
> Phase 1 provides the bridge we need. We can now safely proceed to **Phase 2: Extending the RBAC system**. Phase 2 involves migrating away from the legacy `user.is_admin` boolean to the newly created `user_app_permission` entity.

Ready to begin Phase 2?
