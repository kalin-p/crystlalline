PRAGMA foreign_keys=OFF;

CREATE TABLE temp_files (file UNIQUE PRIMARY KEY, title text, hash NOT NULL, atime NOT NULL, mtime NOT NULL);
CREATE TABLE temp_nodes (id text NOT NULL PRIMARY KEY, file text NOT NULL, level NOT NULL, pos integer NOT NULL, todo text, priority text, scheduled text, deadline text, title text, properties text, olp text, FOREIGN KEY (file) REFERENCES temp_files (file) ON DELETE CASCADE);
CREATE TABLE temp_aliases (node_id text NOT NULL, alias text, FOREIGN KEY (node_id) REFERENCES temp_nodes (id) ON DELETE CASCADE);
CREATE TABLE temp_citations (node_id text NOT NULL, cite_key text NOT NULL, pos integer NOT NULL, properties text, FOREIGN KEY (node_id) REFERENCES temp_nodes (id) ON DELETE CASCADE);
CREATE TABLE temp_refs (node_id text NOT NULL, ref text NOT NULL, type text NOT NULL, FOREIGN KEY (node_id) REFERENCES temp_nodes (id) ON DELETE CASCADE);
CREATE TABLE temp_tags (node_id text NOT NULL, tag text, FOREIGN KEY (node_id) REFERENCES temp_nodes (id) ON DELETE CASCADE);
CREATE TABLE temp_links (pos integer NOT NULL, source text NOT NULL, dest text NOT NULL, type text NOT NULL, properties text NOT NULL, FOREIGN KEY (source) REFERENCES temp_nodes (id) ON DELETE CASCADE);

INSERT INTO temp_files (file, title, hash, atime, mtime)
SELECT file, title, hash, atime, mtime
FROM files;

INSERT INTO temp_nodes (id, file, level, pos, todo, priority, scheduled, deadline, title, properties, olp)
SELECT id, file, level, pos, todo, priority, scheduled, deadline, title, properties, olp
FROM nodes; 

INSERT INTO temp_aliases (node_id, alias)
SELECT node_id, alias
FROM aliases;

INSERT INTO temp_citations (node_id, cite_key, pos, properties)
SELECT node_id, cite_key, pos, properties
FROM citations;

INSERT INTO temp_refs (node_id, ref, type)
SELECT node_id, ref, type
FROM refs;

INSERT INTO temp_tags (node_id, tag)
SELECT node_id, tag
FROM tags;

INSERT INTO temp_links (pos, source, dest, type, properties)
SELECT pos, source, dest, type, properties
FROM links;

DROP TABLE files;
DROP TABLE nodes;
DROP TABLE aliases;
DROP TABLE citations;
DROP TABLE refs;
DROP TABLE tags;
DROP TABLE links;

ALTER TABLE temp_files RENAME TO files;
ALTER TABLE temp_nodes RENAME TO nodes;
ALTER TABLE temp_aliases RENAME TO aliases;
ALTER TABLE temp_citations RENAME TO citations;
ALTER TABLE temp_refs RENAME TO refs;
ALTER TABLE temp_tags RENAME TO tags;
ALTER TABLE temp_links RENAME TO links;

CREATE INDEX alias_node_id ON aliases (node_id);
CREATE INDEX refs_node_id ON refs (node_id);
CREATE INDEX tags_node_id ON tags (node_id);

