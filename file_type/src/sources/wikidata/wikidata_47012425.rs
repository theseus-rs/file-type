use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012425: FileFormat = FileFormat {
    id: 47_012_425,
    puid: "wikidata/47012425",
    name: "Microsoft Visual FoxPro Table file format",
    extensions: &["dbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
