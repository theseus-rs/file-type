use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857392: FileFormat = FileFormat {
    id: 105_857_392,
    puid: "wikidata/105857392",
    name: "Java Web Start application descriptor",
    extensions: &["jnlp"],
    media_types: &["text.xnk"],
    internal_signatures: &[],
    related_formats: &[],
};
