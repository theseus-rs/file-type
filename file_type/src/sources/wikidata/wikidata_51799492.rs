use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51799492: FileFormat = FileFormat {
    id: 51_799_492,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet for DOS, versions 1-4",
    extensions: &["wkq", "wq1"],
    media_types: &["application/x-quattro-pro"],
    internal_signatures: &[],
    related_formats: &[],
};
