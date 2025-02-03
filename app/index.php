<?php

require_once "libonebrc.php";
$libOneBrc = new LibOneBrc();

$filename = "./rust/measurements.txt";

$result = json_decode($libOneBrc->run($filename));

echo "{" . PHP_EOL;
$isFirstRow = true;
foreach ($result as $key => $value) {
    if (!$isFirstRow) {
        echo "," . PHP_EOL;
    } else {
        $isFirstRow = false;
    }

    echo "\t" . $value->city . '=' . $value->max / 10 . '/' . $value->min / 10 . '/' . round($value->sum / $value->count / 10, 1);
}
echo PHP_EOL . "}" . PHP_EOL;

