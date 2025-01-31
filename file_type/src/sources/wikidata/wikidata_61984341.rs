use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61984341: FileFormat = FileFormat {
    id: 61_984_341,
    puid: "wikidata/61984341",
    name: "Microsoft Visual FoxPro database container (memo files)",
    extensions: &["dct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
