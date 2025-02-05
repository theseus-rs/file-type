use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119013900: FileFormat = FileFormat {
    id: 119_013_900,
    source_type: SourceType::Wikidata,
    name: "Binspector grammar",
    extensions: &["bfft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
