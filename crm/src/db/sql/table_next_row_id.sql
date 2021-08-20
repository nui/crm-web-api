-- PREPARE table_next_row_id(text, text) AS
SELECT nextval((SELECT pg_get_serial_sequence($1, $2)));