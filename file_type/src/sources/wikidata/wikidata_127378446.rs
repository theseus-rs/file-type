use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127378446: FileFormat = FileFormat {
    id: 127_378_446,
    puid: "wikidata/127378446",
    name: "GLSL file",
    extensions: &["glsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
