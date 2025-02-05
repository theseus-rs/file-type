use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049547: FileFormat = FileFormat {
    id: 28_049_547,
    source_type: SourceType::Wikidata,
    name: "STAD image",
    extensions: &["pac", "seq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
