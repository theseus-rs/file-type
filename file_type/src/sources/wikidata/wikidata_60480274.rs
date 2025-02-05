use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60480274: FileFormat = FileFormat {
    id: 60_480_274,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for Windows, version 6",
    extensions: &["wb2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
