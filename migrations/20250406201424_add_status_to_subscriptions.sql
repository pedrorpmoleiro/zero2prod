-- Add optional status column to subscriptions
ALTER TABLE subscriptions
    ADD COLUMN status TEXT NULL;
