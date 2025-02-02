use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551274: FileFormat = FileFormat {
    id: 28_551_274,
    source_type: SourceType::Wikidata,
    name: "Adobe Arbitrary Map File",
    extensions: &["amp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
