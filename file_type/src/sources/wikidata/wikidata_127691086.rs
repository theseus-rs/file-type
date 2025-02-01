use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127691086: FileFormat = FileFormat {
    id: 127_691_086,
    puid: "wikidata/127691086",
    name: "Dart file",
    extensions: &["dart"],
    media_types: &["text/x-dart"],
    internal_signatures: &[],
    related_formats: &[],
};
