# mao = Box("MAO", links=["dog"])
#     hairy = Box("america", links=["Boxy hair"])
#     mao.add("log", hairy)
#     Box("log").link(mao)

#     print(mao.get("lo"))

#     math_class = Box("Math 101")
#     art_class = Box("Art History")

#     student_bob = Box("Bob")
#     student_eve = Box("Eve")

#     # Both classes 'add' Bob (Many classes have the same student)
#     math_class.add(student_bob, student_eve)
#     art_class.add(student_bob) 

#     # Bob is now part of two boxes simultaneously
#     print(f"M:N Math -> {math_class}")
#     print(f"M:N Art -> {art_class}")

#     a = Box("a")
#     b = Box("b")
#     a.add(b)
#     b.add(a)

#     print(math_class in a)  # 💥 infinite recursion

#     print(a.get("some_data"))
    # def _get(self, item, seen) -> None | list["Box"] | "Box":
    #     if self.id in seen:
    #         return None # Prevent RecursionError by stopping if seen already
    #     seen.add(self.id) 

    #     try:
    #         item = Box.convert_to_box(item)
    #     except TypeError:
    #         # Contained item cannot be converted to box from UUID due to being data/unknown UUID.
    #         # Checks data against item and does recursion if that is false.
    #         matches = []
    #         for child in self._children:
    #             if child.data == item:
    #                 matches.append(child)

    #             descendant_matches = child._get(item, seen)
    #             if descendant_matches:
    #                 matches.extend(list(descendant_matches))

    #         match matches:
    #             case []:
    #                 return None
    #             case [match]: 
    #                 return match
    #             case _:
    #                 return matches
    #     else:
    #         # Item is Box, so there can only be one match with same UUID, using next to stop evaluating when found.
    #         # Checks item IDs and does recursion if that is false
    #         return next((child for child in self._children if child.id == item.id or child._get(item, seen)), None)
# from deebee import Box
# # You may also construct `Box` simply through Box("Cat", ["American Shorthair"])
# canis = Box.create("Canis Genus", ["Coyote", "Ethiopian Wolf", "Red Wolf", "Gray Wolf"]) # Second argument is sugar for `add` as shown below
# canis.find("Gray Wolf")[0].add("Domestic Dog")


# Box.create("Cat", ["American Shorthair"])
# felis = Box("Felis")
# Box.master_search("Cat").link(felis) # Master search searches the entire database (no variable needed!) Link is the inverse of `add` and can also accept a string.
# felis.add("Jungle Cat", "African Wildcat", "Sand Cat")
# shorthair = felis.find("American Shorthair")[0]

# shorthair.add("Pumpkin", "Luna", "Lil' Wayne") # Get searches all descendants by default for a match.
# shorthair.add("Luna") # Oh no! Two cats with the same name? What will you do?
# print(shorthair.find_all("Luna")[1]) # Crisis averted. Find_all returns a list.
# luna_2_id = shorthair.find_all("Luna")[1].id # `find_all` should return descendants in a primogeniture-like order. Let's save the second Luna's UUID just in case.

# for shorthair_cat in shorthair:
#     print(f"Hello, {shorthair_cat}!")

# # Now, the grand finale!
# felidae_id = felis.link("Felidae").id
# Box.master_find(felidae_id).link("Feliformia") # ID lookups are faster as they are a simple dict lookup rather than a loop.
# Box.master_find("Feliformia").link("Carnivora")
# Box.master_search("Carnivora").add("Caniformia")
# Box.master_search("Caniformia").add("Canidae")
# Box.master_search("Canidae").add("Canis")

# # Bye bye!
