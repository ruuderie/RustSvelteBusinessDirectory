-- Create directory types
INSERT INTO directory_type (id, name, description, created_at, updated_at)
VALUES
  (gen_random_uuid(), 'Transportation & Logistics', 'Directory for transportation and logistics services', NOW(), NOW()),
  (gen_random_uuid(), 'Automotive Sales', 'Directory for automotive sales and dealerships', NOW(), NOW()),
  (gen_random_uuid(), 'Construction & Contracting', 'Directory for construction and contracting services', NOW(), NOW()),
  (gen_random_uuid(), 'Beauty & Personal Care', 'Directory for beauty and personal care services', NOW(), NOW()),
  (gen_random_uuid(), 'Financial Services', 'Directory for various financial and lending services', NOW(), NOW());

-- Create directories for Transportation & Logistics
INSERT INTO directory (id, directory_type_id, name, domain, description, created_at, updated_at)
SELECT
  gen_random_uuid(),
  dt.id,
  d.name,
  d.domain,
  d.description,
  NOW(),
  NOW()
FROM
  (VALUES
    ('Global Logistics Network', 'globallogisticsnetwork.com', 'Connecting logistics professionals worldwide'),
    ('Freight Connect', 'freightconnect.com', 'Your hub for freight and shipping solutions'),
    ('Supply Chain Directory', 'supplychaindirectory.com', 'Comprehensive directory for supply chain management')
  ) AS d(name, domain, description),
  directory_type dt
WHERE dt.name = 'Transportation & Logistics';

-- Create directories for Automotive Sales
INSERT INTO directory (id, directory_type_id, name, domain, description, created_at, updated_at)
SELECT
  gen_random_uuid(),
  dt.id,
  d.name,
  d.domain,
  d.description,
  NOW(),
  NOW()
FROM
  (VALUES
    ('Auto Dealer Hub', 'autodealerhub.com', 'Connecting car buyers with trusted dealerships'),
    ('CarSales Pro', 'carsalespro.com', 'Professional network for automotive sales'),
    ('Vehicle Marketplace', 'vehiclemarketplace.com', 'Your one-stop shop for all vehicle needs')
  ) AS d(name, domain, description),
  directory_type dt
WHERE dt.name = 'Automotive Sales';

-- Create directories for Construction & Contracting
INSERT INTO directory (id, directory_type_id, name, domain, description, created_at, updated_at)
SELECT
  gen_random_uuid(),
  dt.id,
  d.name,
  d.domain,
  d.description,
  NOW(),
  NOW()
FROM
  (VALUES
    ('Builder Connect', 'builderconnect.com', 'Connecting construction professionals and clients'),
    ('Contractor Network', 'contractornetwork.com', 'Find reliable contractors for your projects'),
    ('Construction Industry Directory', 'constructionindustrydirectory.com', 'Comprehensive directory for the construction industry')
  ) AS d(name, domain, description),
  directory_type dt
WHERE dt.name = 'Construction & Contracting';

-- Create directories for Beauty & Personal Care
INSERT INTO directory (id, directory_type_id, name, domain, description, created_at, updated_at)
SELECT
  gen_random_uuid(),
  dt.id,
  d.name,
  d.domain,
  d.description,
  NOW(),
  NOW()
FROM
  (VALUES
    ('Salon & Spa Directory', 'salonspadirectory.com', 'Find top-rated salons and spas near you'),
    ('Beauty Pro Network', 'beautypronetwork.com', 'Connecting beauty professionals and clients'),
    ('Style Finder', 'stylefinder.com', 'Discover your perfect style and beauty services')
  ) AS d(name, domain, description),
  directory_type dt
WHERE dt.name = 'Beauty & Personal Care';

-- Create directories for Financial Services (Lending)
INSERT INTO directory (id, directory_type_id, name, domain, description, created_at, updated_at)
SELECT
  gen_random_uuid(),
  dt.id,
  d.name,
  d.domain,
  d.description,
  NOW(),
  NOW()
FROM
  (VALUES
    ('Real Estate Loan Finder', 'realestateLoanfinder.com', 'Connect with lenders specializing in real estate financing'),
    ('Business Loan Network', 'businessloannetwork.com', 'Find the right business loan for your company''s needs'),
    ('Acquisition Finance Directory', 'acquisitionfinancedirectory.com', 'Specialized lending options for mergers and acquisitions')
  ) AS d(name, domain, description),
  directory_type dt
WHERE dt.name = 'Financial Services';