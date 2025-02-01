use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854136: FileFormat = FileFormat {
    id: 105_854_136,
    puid: "wikidata/105854136",
    name: "PS/2 MicroChannel Adapter Description File (with rem)",
    extensions: &["adf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
