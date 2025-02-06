use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59820830: FileFormat = FileFormat {
    id: 59_820_830,
    source_type: SourceType::Wikidata,
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
