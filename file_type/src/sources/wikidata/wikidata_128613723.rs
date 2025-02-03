use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128613723: FileFormat = FileFormat {
    id: 128_613_723,
    source_type: SourceType::Wikidata,
    name: "AspectJ file format",
    extensions: &["aj"],
    media_types: &["text/x-aspectj"],
    internal_signatures: &[],
    related_formats: &[],
};
