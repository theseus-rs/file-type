use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28551337: FileFormat = FileFormat {
    id: 28_551_337,
    source_type: SourceType::Wikidata,
    name: "Adobe Duotone Options File",
    extensions: &["ado"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
