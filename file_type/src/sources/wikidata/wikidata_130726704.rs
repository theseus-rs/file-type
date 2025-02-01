use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130726704: FileFormat = FileFormat {
    id: 130_726_704,
    puid: "wikidata/130726704",
    name: "SARL source code file",
    extensions: &["sarl"],
    media_types: &["text/x-sarl"],
    internal_signatures: &[],
    related_formats: &[],
};
