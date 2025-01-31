use crate::format::FileFormat;

pub(crate) const LINGUIST_293: FileFormat = FileFormat {
    id: 293,
    puid: "linguist/293",
    name: "PowerShell",
    extensions: &["ps1", "psd1", "psm1"],
    media_types: &["application/x-powershell"],
    internal_signatures: &[],
    related_formats: &[],
};
