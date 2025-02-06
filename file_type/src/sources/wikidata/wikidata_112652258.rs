use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652258: FileFormat = FileFormat {
    id: 112_652_258,
    source_type: SourceType::Wikidata,
    name: "3ds Max Characters",
    extensions: &["chr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
