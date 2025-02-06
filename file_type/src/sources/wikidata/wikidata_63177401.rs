use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63177401: FileFormat = FileFormat {
    id: 63_177_401,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Spreadsheet for Macintosh, version 4",
    extensions: &["wks"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
