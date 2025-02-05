use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62625183: FileFormat = FileFormat {
    id: 62_625_183,
    source_type: SourceType::Wikidata,
    name: "PowerShell script",
    extensions: &["ps1", "psm1"],
    media_types: &["application/x-powershell", "text/x-powershell"],
    signatures: &[],
    related_formats: &[],
};
