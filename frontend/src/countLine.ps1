$results = Get-ChildItem -Directory | ForEach-Object {
    $lines = Get-ChildItem $_.FullName -Recurse -Include *.ts, *.vue, *.js -File |
        ForEach-Object {
            (Get-Content $_ | Measure-Object -Line).Lines
        } |
        Measure-Object -Sum

    [PSCustomObject]@{
        Folder = $_.Name
        Lines  = $lines.Sum
    }
}

# Show per-folder results
$results

# Show total at the end
$total = ($results.Lines | Measure-Object -Sum).Sum
"`nTotal Lines: $total"