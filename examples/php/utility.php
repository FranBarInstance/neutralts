<?php


    function negotiate_language($available_languages = ['en']) {
        $accept_language = getenv('HTTP_ACCEPT_LANGUAGE');

        if (isset($_GET['lang']) && in_array($_GET['lang'], $available_languages)) {
            return $_GET['lang'];
        }

        if (isset($_COOKIE['lang']) && in_array($_COOKIE['lang'], $available_languages)) {
            return $_COOKIE['lang'];
        }

        if (empty($accept_language)) {
            return reset($available_languages);
        }

        preg_match_all('/([a-z]{1,8}(-[a-z]{1,8})?)\s*(;\s*q\s*=\s*(0(\.\d{1,3})?|1(\.0{1,3})?))?\s*/i', $accept_language, $matches);

        $languages = [];
        foreach ($matches[1] as $index => $language) {
            $quality = isset($matches[4][$index]) ? floatval($matches[4][$index]) : 1.0;
            if (!isset($languages[$language])) {
                $languages[$language] = $quality;
            } else {
                $languages[$language] = max($languages[$language], $quality);
            }
        }

        uksort($languages, function ($lang1, $lang2) use ($languages) {
            return $languages[$lang2] <=> $languages[$lang1];
        });

        $preferred_languages = array_intersect_key($languages, array_flip($available_languages));

        reset($preferred_languages);
        return key($preferred_languages);
    }