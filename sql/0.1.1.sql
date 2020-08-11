CREATE TABLE `api_user_secret` (
    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
    `app_code` varchar(32) NOT NULL DEFAULT '',
    `app_secret` varchar(32) NOT NULL DEFAULT '',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `IDX_APP_CODE` (`app_code`)
) ENGINE=InnoDB AUTO_INCREMENT=100015 DEFAULT CHARSET=utf8mb4;

CREATE TABLE `admin` (
    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
    `username` varchar(32) NOT NULL DEFAULT '',
    `passwd` char(32) NOT NULL DEFAULT '',
    `salt` char(12) NOT NULL DEFAULT '',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `IDX_USERNAME` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;

CREATE TABLE `admin_access_token` (
    `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
    `admin_id` int(11) unsigned NOT NULL DEFAULT '0',
    `access_token` varchar(32) NOT NULL DEFAULT '',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `IDX_ADMIN_ID` (`admin_id`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4;
