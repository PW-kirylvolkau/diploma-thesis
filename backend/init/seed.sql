insert into universities (name) values
('university 1');

insert into courses (name, tile_url, is_public) values
('math', 'invalid_url', TRUE),
('language', 'invalid_url', TRUE);

insert into lessons (name, text, type, resource_url) values
('math 1', 'this is a lesson 1 from math', 'ATTACHMENT', 'https://diplomablob.blob.core.windows.net/lesson-attachments/KirylVolkau_CV.pdf'),
('language 1', 'this is a lesson from language', 'VIDEO', 'https://diplomablob.blob.core.windows.net/lesson-videos/057c4384-70c2-49f5-97fc-a39a3998789c.mp4'),
('math 2', 'this is a lesson 2 from math', 'TEXT', NULL);

insert into university_courses (uni_id, course_id) values
(1, 1),
(1, 2);

-- TODO: refactor to operate on sequences
insert into course_lessons (course_id, lesson_id, seq) values
(1, 1, 1),
(2, 1, 1),
(1, 3, 2);