use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55758993: FileFormat = FileFormat {
    id: 55_758_993,
    puid: "wikidata/55758993",
    name: "Microsoft Program Database file format, version 7",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
