use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128613723: FileFormat = FileFormat {
    id: 128_613_723,
    puid: "wikidata/128613723",
    name: "AspectJ file format",
    extensions: &["aj"],
    media_types: &["text/x-aspectj"],
    internal_signatures: &[],
    related_formats: &[],
};
