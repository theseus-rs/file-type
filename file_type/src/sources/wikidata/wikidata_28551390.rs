use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28551390: FileFormat = FileFormat {
    id: 28_551_390,
    source_type: SourceType::Wikidata,
    name: "Adobe Selective Color File",
    extensions: &["asv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
