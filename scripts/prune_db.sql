-- Generated by ChatGPT
-- run in psql directly
SELECT 'DROP DATABASE ' || quote_ident(datname) || ';'
FROM pg_database
WHERE datistemplate = false
  AND datname NOT IN ('postgres', 'newsletter')
\gexec
