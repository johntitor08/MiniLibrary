[Setup]
AppName=Mini Library
AppVersion=1.0
DefaultDirName={commonpf32}\MiniLibrary
DefaultGroupName=Mini Library
OutputBaseFilename=MiniLibrarySetup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
OutputDir=Output

[Files]
Source: "target\release\mini-library.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "library.json"; DestDir: "{app}"; Flags: ignoreversion
Source: "covers\*"; DestDir: "{app}\covers"; Flags: recursesubdirs

[Icons]
Name: "{group}\Mini Library"; Filename: "{app}\mini-library.exe"
