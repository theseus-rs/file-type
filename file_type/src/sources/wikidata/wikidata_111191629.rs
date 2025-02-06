use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111191629: FileFormat = FileFormat {
    id: 111_191_629,
    source_type: SourceType::Wikidata,
    name: "Viewpoint format",
    extensions: &["vet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
