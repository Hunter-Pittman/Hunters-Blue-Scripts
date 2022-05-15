# Comments


function Show-Menu {
    Clear-Host
    Write-Host "====== Registry Monitor ======"

    Write-Host "1: Generate inital snapshots"
    Write-Host "2: Compare current registry state"
    Write-Host "q: Exit this program"
    $userinput = Read-Host "Enter (1-3, or q): "
    switch ($userinput) {
        1 { Clear-Host Initial-Snapshot }
        2 { Current-Compare }
        q {}
    }
    
}


function Initial-Snapshot {
    dir -rec -erroraction ignore HKLM:\ | % name > Base-HKLM.txt
    dir -rec -erroraction ignore HKCU:\ | % name > Base-HKCU.txt
    Write-Host "Inital snapshot complete"
}

function Current-Compare {

    $HKLMPathCheck = Test-Path -Path ./Base-HKLM.txt -PathType Leaf
    $HKCUPathCheck = Test-Path -Path ./Base-HKCU.txt -PathType Leaf

    if ($HKLMPathCheck -eq $false) {
        Write-Host "HKLM Not here"
    }

    if ($HKCUPathCheck -eq $false) {
        Write-Host "HKCU Not here"
    }

    $CurrentDateTime = Get-Date -Format "yyyyMMddHHmm"
    
    $HKLMLogName = "HKLM_" + $CurrentDateTime
    $HKCULogName = "HKCU_" + $CurrentDateTime

    dir -rec -erroraction ignore HKLM:\ | % name > $HKLMLogName
    dir -rec -erroraction ignore HKCU:\ | % name > $HKCULogName

    Compare-Object (Get-Content -Path ./$HKLMLogName) -DifferenceObject (Get-Content -Path ./$HKCULogName)

}

Current-Compare