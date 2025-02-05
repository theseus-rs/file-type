use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837034: FileFormat = FileFormat {
    id: 108_837_034,
    source_type: SourceType::Wikidata,
    name: "Nero Video-CD Compilation",
    extensions: &["nrv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
