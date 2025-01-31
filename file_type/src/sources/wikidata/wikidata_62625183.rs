use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62625183: FileFormat = FileFormat {
    id: 62_625_183,
    puid: "wikidata/62625183",
    name: "PowerShell script",
    extensions: &["ps1", "ps1", "psm1", "psm1"],
    media_types: &[
        "application/x-powershell",
        "application/x-powershell",
        "text/x-powershell",
        "text/x-powershell",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
