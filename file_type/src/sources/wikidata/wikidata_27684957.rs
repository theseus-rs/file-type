use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27684957: FileFormat = FileFormat {
    id: 27_684_957,
    puid: "wikidata/27684957",
    name: "Microsoft Publisher Pack and Go file format",
    extensions: &["puz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
