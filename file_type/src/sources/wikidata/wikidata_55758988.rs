use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55758988: FileFormat = FileFormat {
    id: 55_758_988,
    puid: "wikidata/55758988",
    name: "Microsoft Program Database file format, version 2",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
