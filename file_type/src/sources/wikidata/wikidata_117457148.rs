use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117457148: FileFormat = FileFormat {
    id: 117_457_148,
    puid: "wikidata/117457148",
    name: "Microsoft Access Encrypted Database File 2.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
