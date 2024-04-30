CREATE TABLE links (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    target_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE
)