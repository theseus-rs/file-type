use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009607: FileFormat = FileFormat {
    id: 111_009_607,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Letterhead File format",
    extensions: &["let"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
