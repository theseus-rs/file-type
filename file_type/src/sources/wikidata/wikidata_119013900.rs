use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119013900: FileFormat = FileFormat {
    id: 119_013_900,
    source_type: SourceType::Wikidata,
    name: "Binspector grammar",
    extensions: &["bfft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
