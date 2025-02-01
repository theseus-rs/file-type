use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127784231: FileFormat = FileFormat {
    id: 127_784_231,
    puid: "wikidata/127784231",
    name: "Logtalk source file",
    extensions: &["lgt"],
    media_types: &["text/x-logtalk"],
    internal_signatures: &[],
    related_formats: &[],
};
