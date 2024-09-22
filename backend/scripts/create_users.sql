DO $$
DECLARE
    directory_id UUID;
    account_id UUID;
    user_id UUID;
    profile_id UUID;
    user_account_id UUID;
    username TEXT;
    email TEXT;
    display_name TEXT;
    business_name TEXT;
    phone_number TEXT;
    website TEXT;
    first_name TEXT;
    last_name TEXT;
    domain TEXT;
BEGIN
    -- Array of realistic first names
    first_name := (ARRAY['James', 'John', 'Robert', 'Michael', 'William', 'David', 'Richard', 'Joseph', 'Thomas', 'Charles', 'Mary', 'Patricia', 'Jennifer', 'Linda', 'Elizabeth', 'Barbara', 'Margaret', 'Susan', 'Dorothy', 'Lisa'])[floor(random() * 20 + 1)];
    
    -- Array of realistic last names
    last_name := (ARRAY['Smith', 'Johnson', 'Williams', 'Brown', 'Jones', 'Garcia', 'Miller', 'Davis', 'Rodriguez', 'Martinez', 'Hernandez', 'Lopez', 'Gonzalez', 'Wilson', 'Anderson', 'Thomas', 'Taylor', 'Moore', 'Jackson', 'Martin'])[floor(random() * 20 + 1)];

    -- Loop through each directory
    FOR directory_id, business_name IN 
        VALUES 
        ('d05510f1-8417-4c16-b1c8-543206903ffc', 'Real Estate Loan Finder'),
        ('0a529c09-3a95-4e30-9875-e8544928cd9c', 'Business Loan Network'),
        ('152f995e-3109-45a8-86d8-924697e207f2', 'Acquisition Finance Directory'),
        ('e6c78333-35d1-45ea-bfaf-878ee1520295', 'Salon & Spa Directory'),
        ('34719ace-bc2b-427b-b17c-4e7e88d8475a', 'Beauty Pro Network'),
        ('77f4b52b-8222-4a5c-8cda-a77505e15454', 'Style Finder'),
        ('09f58f7e-fa3b-4c33-90fc-d7c58e7676b5', 'Builder Connect'),
        ('1634cd8a-f87e-4f04-9712-8cbf8922be0a', 'Contractor Network'),
        ('c928ab37-f599-411e-a793-97cb4633c822', 'Construction Industry Directory'),
        ('fcb166e9-297b-4dd0-907e-6aacc1f109f8', 'Auto Dealer Hub'),
        ('62952f16-35d0-49cc-b8c9-c35b3d741b5d', 'CarSales Pro'),
        ('339478d3-4128-41c4-846a-de381970b1df', 'Vehicle Marketplace'),
        ('83863dd0-a654-4d7c-a562-91782873424e', 'Global Logistics Network'),
        ('e7f8b5c7-b401-457e-9732-ccad64fa3ca2', 'Freight Connect'),
        ('29e7a33c-48e1-4867-8eed-930fc94eb4a4', 'Supply Chain Directory')
    LOOP
        -- Create 3 users for each directory
        FOR i IN 1..3 LOOP
            -- Generate user data
            first_name := (ARRAY['James', 'John', 'Robert', 'Michael', 'William', 'David', 'Richard', 'Joseph', 'Thomas', 'Charles', 'Mary', 'Patricia', 'Jennifer', 'Linda', 'Elizabeth', 'Barbara', 'Margaret', 'Susan', 'Dorothy', 'Lisa'])[floor(random() * 20 + 1)];
            last_name := (ARRAY['Smith', 'Johnson', 'Williams', 'Brown', 'Jones', 'Garcia', 'Miller', 'Davis', 'Rodriguez', 'Martinez', 'Hernandez', 'Lopez', 'Gonzalez', 'Wilson', 'Anderson', 'Thomas', 'Taylor', 'Moore', 'Jackson', 'Martin'])[floor(random() * 20 + 1)];
            
            username := lower(first_name || '.' || last_name || floor(random() * 100)::text);
            display_name := first_name || ' ' || last_name;
            
            domain := (ARRAY['gmail.com', 'yahoo.com', 'outlook.com', 'hotmail.com', 'example.com'])[floor(random() * 5 + 1)];
            email := username || '@' || domain;
            
            -- Generate phone number
            phone_number := '+1 ' || 
                            LPAD(CAST(floor(random() * 900 + 100) AS TEXT), 3, '0') || '-' ||
                            LPAD(CAST(floor(random() * 900 + 100) AS TEXT), 3, '0') || '-' ||
                            LPAD(CAST(floor(random() * 9000 + 1000) AS TEXT), 4, '0');

            -- Generate website
            website := 'www.' || lower(regexp_replace(business_name, '\s+', '', 'g')) || '.com';

            -- Insert user
            INSERT INTO "user" (id, username, email, password_hash, is_admin, is_active, created_at, updated_at)
            VALUES (gen_random_uuid(), username, email, 'hashed_password', false, true, NOW(), NOW())
            RETURNING id INTO user_id;

            -- Insert account
            INSERT INTO account (id, directory_id, name, is_active, created_at, updated_at)
            VALUES (gen_random_uuid(), directory_id, last_name || ' ' || business_name || ' Account', true, NOW(), NOW())
            RETURNING id INTO account_id;

            -- Insert user_account
            INSERT INTO user_account (id, user_id, account_id, role, is_active, created_at, updated_at)
            VALUES (gen_random_uuid(), user_id, account_id, 
                    CASE WHEN i = 1 THEN 'Owner'
                         WHEN i = 2 THEN 'Admin'
                         ELSE 'Member'
                    END, 
                    true, NOW(), NOW())
            RETURNING id INTO user_account_id;

            -- Insert profile
            INSERT INTO profile (
                id, account_id, directory_id, profile_type, display_name, contact_info, 
                business_name, business_address, business_phone, business_website, 
                additional_info, is_active, created_at, updated_at
            )
            VALUES (
                gen_random_uuid(), account_id, directory_id, 
                CASE WHEN random() < 0.7 THEN 'Business' ELSE 'Individual' END,
                display_name, email, 
                CASE WHEN random() < 0.7 THEN last_name || ' ' || business_name ELSE NULL END,
                CASE WHEN random() < 0.7 THEN 
                    floor(random() * 9999 + 1)::text || ' ' || 
                    (ARRAY['Main', 'Oak', 'Pine', 'Maple', 'Cedar'])[floor(random() * 5 + 1)] || ' ' ||
                    (ARRAY['St', 'Ave', 'Blvd', 'Rd', 'Ln'])[floor(random() * 5 + 1)] || ', ' ||
                    (ARRAY['New York', 'Los Angeles', 'Chicago', 'Houston', 'Phoenix', 'Philadelphia', 'San Antonio', 'San Diego', 'Dallas', 'San Jose'])[floor(random() * 10 + 1)] || ', ' ||
                    (ARRAY['NY', 'CA', 'IL', 'TX', 'AZ', 'PA', 'TX', 'CA', 'TX', 'CA'])[floor(random() * 10 + 1)]
                ELSE NULL END,
                CASE WHEN random() < 0.7 THEN phone_number ELSE NULL END,
                CASE WHEN random() < 0.5 THEN website ELSE NULL END,
                json_build_object(
                    'years_of_experience', floor(random() * 30 + 1),
                    'specialties', ARRAY[
                        (ARRAY['Residential', 'Commercial', 'Industrial', 'Luxury', 'Foreclosures'])[floor(random() * 5 + 1)],
                        (ARRAY['Sales', 'Rentals', 'Property Management', 'Investment', 'Development'])[floor(random() * 5 + 1)]
                    ]
                ),
                true, NOW(), NOW()
            )
            RETURNING id INTO profile_id;

        END LOOP;
    END LOOP;
END $$;