use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28009476: FileFormat = FileFormat {
    id: 28_009_476,
    source_type: SourceType::Wikidata,
    name: "SCF",
    extensions: &["scf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
