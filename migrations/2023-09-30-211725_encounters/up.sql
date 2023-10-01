-- Your SQL goes here
CREATE TABLE encounters (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    is_dangerous BOOLEAN NOT NULL,
    is_abused BOOLEAN NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    image_urls TEXT,
    created_at TIMESTAMP default CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO encounters (user_id, title, description, is_dangerous, is_abused, latitude, longitude, image_urls)
VALUES (
    1, 
    'wonsz rzeczny', 
    'tu du du tu du du jest niebezpieczny', 
    true,
    false,
    51.99074828576804, 
    21.234140746236903, 
    '["https://img.besty.pl/images/394/41/3944145.jpg,https://viosna.pl/wp-content/uploads/10102009855-255x300.png,https://s3.eu-central-1.amazonaws.com/cdn.koty.pl/baby_cat_png_12_1_0e0fc56629.png"]'
);