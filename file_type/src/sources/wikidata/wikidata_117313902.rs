use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117313902: FileFormat = FileFormat {
    id: 117_313_902,
    puid: "wikidata/117313902",
    name: "Clear Text CGM",
    extensions: &["ctm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
