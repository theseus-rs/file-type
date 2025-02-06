use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205760: FileFormat = FileFormat {
    id: 28_205_760,
    source_type: SourceType::Wikidata,
    name: "Borland Graphics Interface image",
    extensions: &["bgi", "icn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
