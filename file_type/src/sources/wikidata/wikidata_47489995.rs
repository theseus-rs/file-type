use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47489995: FileFormat = FileFormat {
    id: 47_489_995,
    source_type: SourceType::Wikidata,
    name: "Adobe FrameMaker Document, version 6",
    extensions: &["fm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
