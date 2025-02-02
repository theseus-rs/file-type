use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_68480995: FileFormat = FileFormat {
    id: 68_480_995,
    source_type: SourceType::Wikidata,
    name: "Kingsoft PowerWord Dictionary",
    extensions: &["dic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
