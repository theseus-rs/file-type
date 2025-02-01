use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538988: FileFormat = FileFormat {
    id: 47_538_988,
    puid: "wikidata/47538988",
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
