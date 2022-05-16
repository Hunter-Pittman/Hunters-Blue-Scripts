function Overall-Info {
    $SystemInfo = Get-ComputerInfo
    $ExecutionPolicy = Get-ExecutionPolicy

    $EssentialInfo = [PSCustomObject]@{
        ComputerName    = $SystemInfo.CsName
        ProductVersion  = $SystemInfo.WindowsProductName
        OsVersion       = $SystemInfo.OsVersion
        NetworkAdapters = $SystemInfo.CsNetworkAdapters
        UserName        = $SystemInfo.CsUserName
        NumUsers        = $SystemInfo.OsNumberOfUsers
        LogonServer     = $SystemInfo.LogonServer
        ExecutionPolicy = $ExecutionPolicy
    }

    if ($SystemInfo.CsPartOfDomain -eq $false) {
        $Domain += Add-Member -MemberType NoteProperty -Name Domain -Value $SystemInfo.CsDomain #broke
    }

    $EssentialInfo = ConvertTo-Json -InputObject $EssentialInfo

    return $EssentialInfo

}


function User-Audit {
    $Users = Get-LocalUser | Select *

    $OrganizedUsers = @{}

    foreach ($item in $Users) {
        
    }


}

Overall-Info