use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009592: FileFormat = FileFormat {
    id: 111_009_592,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Label File format",
    extensions: &["lbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
