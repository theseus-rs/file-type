use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551337: FileFormat = FileFormat {
    id: 28_551_337,
    source_type: SourceType::Wikidata,
    name: "Adobe Duotone Options File",
    extensions: &["ado"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
