use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85621726: FileFormat = FileFormat {
    id: 85_621_726,
    source_type: SourceType::Wikidata,
    name: "PFS:First Choice Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
