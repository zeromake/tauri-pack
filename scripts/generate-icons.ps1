param(
    $in,
	$out
)

$env:path+=";$($PSScriptRoot)\..\bin"

$resize = @(
	("32x32", "32x32.png"),
	("128x128", "128x128.png"),
	("256x256", "128x128@2x.png"),
	("128x128", "icon.ico"),
	("512x512", "icon.png"),
	("30x30", "Square30x30Logo.png"),
	("44x44", "Square44x44Logo.png"),
	("71x71", "Square71x71Logo.png"),
	("89x89", "Square89x89Logo.png"),
	("107x107", "Square107x107Logo.png"),
	("142x142", "Square142x142Logo.png"),
	("150x150", "Square150x150Logo.png"),
	("284x284", "Square284x284Logo.png"),
	("310x310", "Square310x310Logo.png"),
	("50x50", "StoreLogo.png")
)
if ((Get-Command magick -errorAction SilentlyContinue)) {
	foreach ($item in $resize) {
		magick convert -resize "$($item[0])" "$($in)" "$($out)\$($item[1])"
		echo "$($out)\$($item[1]) done"
	}
} else {
	echo "请安装 magick: https://imagemagick.org"
}

if ((Get-Command png2icons -errorAction SilentlyContinue)) {
    png2icons "$($in)" "$($out)\icon" -icns -bc2
	echo "$($out)\icon.icns done"
} else {
	echo "请安装 png2icons: https://github.com/idesis-gmbh/png2icons/releases"
}
